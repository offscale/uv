use std::str::FromStr;
use std::sync::Arc;

use pep440_rs::{Version, VersionParseError};
#[cfg(feature = "pyo3")]
use pyo3::{exceptions::PyValueError, pyclass, pymethods, types::PyAnyMethods, PyResult, Python};

use crate::{MarkerValueString, MarkerValueVersion, StringVersion};

/// The marker values for a python interpreter, normally the current one
///
/// <https://packaging.python.org/en/latest/specifications/dependency-specifiers/#environment-markers>
///
/// Some are `(String, Version)` because we have to support version comparison
#[allow(missing_docs, clippy::unsafe_derive_deserialize)]
#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "pyo3", pyclass(module = "pep508"))]
pub struct MarkerEnvironment {
    #[serde(flatten)]
    inner: Arc<MarkerEnvironmentInner>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
struct MarkerEnvironmentInner {
    implementation_name: String,
    implementation_version: StringVersion,
    os_name: String,
    platform_machine: String,
    platform_python_implementation: String,
    platform_release: String,
    platform_system: String,
    platform_version: String,
    python_full_version: StringVersion,
    python_version: StringVersion,
    sys_platform: String,
}

impl MarkerEnvironment {
    /// Returns of the PEP 440 version typed value of the key in the current environment
    pub fn get_version(&self, key: &MarkerValueVersion) -> &Version {
        match key {
            MarkerValueVersion::ImplementationVersion => &self.implementation_version().version,
            MarkerValueVersion::PythonFullVersion => &self.python_full_version().version,
            MarkerValueVersion::PythonVersion => &self.python_version().version,
        }
    }

    /// Returns of the stringly typed value of the key in the current environment
    pub fn get_string(&self, key: &MarkerValueString) -> &str {
        match key {
            MarkerValueString::ImplementationName => self.implementation_name(),
            MarkerValueString::OsName | MarkerValueString::OsNameDeprecated => self.os_name(),
            MarkerValueString::PlatformMachine | MarkerValueString::PlatformMachineDeprecated => {
                self.platform_machine()
            }
            MarkerValueString::PlatformPythonImplementation
            | MarkerValueString::PlatformPythonImplementationDeprecated
            | MarkerValueString::PythonImplementationDeprecated => {
                self.platform_python_implementation()
            }
            MarkerValueString::PlatformRelease => self.platform_release(),
            MarkerValueString::PlatformSystem => self.platform_system(),
            MarkerValueString::PlatformVersion | MarkerValueString::PlatformVersionDeprecated => {
                self.platform_version()
            }
            MarkerValueString::SysPlatform | MarkerValueString::SysPlatformDeprecated => {
                self.sys_platform()
            }
        }
    }
}

/// APIs for retrieving specific parts of a marker environment.
impl MarkerEnvironment {
    /// Returns the name of the Python implementation for this environment.
    ///
    /// This is equivalent to `sys.implementation.name`.
    ///
    /// Some example values are: `cpython`.
    #[inline]
    pub fn implementation_name(&self) -> &str {
        &self.inner.implementation_name
    }

    /// Returns the Python implementation version for this environment.
    ///
    /// This value is derived from `sys.implementation.version`. See [PEP 508
    /// environment markers] for full details.
    ///
    /// This is equivalent to `sys.implementation.name`.
    ///
    /// Some example values are: `3.4.0`, `3.5.0b1`.
    ///
    /// [PEP 508 environment markers]: https://peps.python.org/pep-0508/#environment-markers
    #[inline]
    pub fn implementation_version(&self) -> &StringVersion {
        &self.inner.implementation_version
    }

    /// Returns the name of the operating system for this environment.
    ///
    /// This is equivalent to `os.name`.
    ///
    /// Some example values are: `posix`, `java`.
    #[inline]
    pub fn os_name(&self) -> &str {
        &self.inner.os_name
    }

    /// Returns the name of the machine for this environment's platform.
    ///
    /// This is equivalent to `platform.machine()`.
    ///
    /// Some example values are: `x86_64`.
    #[inline]
    pub fn platform_machine(&self) -> &str {
        &self.inner.platform_machine
    }

    /// Returns the name of the Python implementation for this environment's
    /// platform.
    ///
    /// This is equivalent to `platform.python_implementation()`.
    ///
    /// Some example values are: `CPython`, `Jython`.
    #[inline]
    pub fn platform_python_implementation(&self) -> &str {
        &self.inner.platform_python_implementation
    }

    /// Returns the release for this environment's platform.
    ///
    /// This is equivalent to `platform.release()`.
    ///
    /// Some example values are: `3.14.1-x86_64-linode39`, `14.5.0`, `1.8.0_51`.
    #[inline]
    pub fn platform_release(&self) -> &str {
        &self.inner.platform_release
    }

    /// Returns the system for this environment's platform.
    ///
    /// This is equivalent to `platform.system()`.
    ///
    /// Some example values are: `Linux`, `Windows`, `Java`.
    #[inline]
    pub fn platform_system(&self) -> &str {
        &self.inner.platform_system
    }

    /// Returns the version for this environment's platform.
    ///
    /// This is equivalent to `platform.version()`.
    ///
    /// Some example values are: `#1 SMP Fri Apr 25 13:07:35 EDT 2014`,
    /// `Java HotSpot(TM) 64-Bit Server VM, 25.51-b03, Oracle Corporation`,
    /// `Darwin Kernel Version 14.5.0: Wed Jul 29 02:18:53 PDT 2015;
    /// root:xnu-2782.40.9~2/RELEASE_X86_64`.
    #[inline]
    pub fn platform_version(&self) -> &str {
        &self.inner.platform_version
    }

    /// Returns the full version of Python for this environment.
    ///
    /// This is equivalent to `platform.python_version()`.
    ///
    /// Some example values are: `3.4.0`, `3.5.0b1`.
    #[inline]
    pub fn python_full_version(&self) -> &StringVersion {
        &self.inner.python_full_version
    }

    /// Returns the version of Python for this environment.
    ///
    /// This is equivalent to `'.'.join(platform.python_version_tuple()[:2])`.
    ///
    /// Some example values are: `3.4`, `2.7`.
    #[inline]
    pub fn python_version(&self) -> &StringVersion {
        &self.inner.python_version
    }

    /// Returns the name of the system platform for this environment.
    ///
    /// This is equivalent to `sys.platform`.
    ///
    /// Some example values are: `linux`, `linux2`, `darwin`, `java1.8.0_51`
    /// (note that `linux` is from Python3 and `linux2` from Python2).
    #[inline]
    pub fn sys_platform(&self) -> &str {
        &self.inner.sys_platform
    }
}

/// APIs for setting specific parts of a marker environment.
impl MarkerEnvironment {
    /// Set the name of the Python implementation for this environment.
    ///
    /// See also [`MarkerEnvironment::implementation_name`].
    #[inline]
    #[must_use]
    pub fn with_implementation_name(mut self, value: impl Into<String>) -> MarkerEnvironment {
        Arc::make_mut(&mut self.inner).implementation_name = value.into();
        self
    }

    /// Set the Python implementation version for this environment.
    ///
    /// See also [`MarkerEnvironment::implementation_version`].
    #[inline]
    #[must_use]
    pub fn with_implementation_version(
        mut self,
        value: impl Into<StringVersion>,
    ) -> MarkerEnvironment {
        Arc::make_mut(&mut self.inner).implementation_version = value.into();
        self
    }

    /// Set the name of the operating system for this environment.
    ///
    /// See also [`MarkerEnvironment::os_name`].
    #[inline]
    #[must_use]
    pub fn with_os_name(mut self, value: impl Into<String>) -> MarkerEnvironment {
        Arc::make_mut(&mut self.inner).os_name = value.into();
        self
    }

    /// Set the name of the machine for this environment's platform.
    ///
    /// See also [`MarkerEnvironment::platform_machine`].
    #[inline]
    #[must_use]
    pub fn with_platform_machine(mut self, value: impl Into<String>) -> MarkerEnvironment {
        Arc::make_mut(&mut self.inner).platform_machine = value.into();
        self
    }

    /// Set the name of the Python implementation for this environment's
    /// platform.
    ///
    /// See also [`MarkerEnvironment::platform_python_implementation`].
    #[inline]
    #[must_use]
    pub fn with_platform_python_implementation(
        mut self,
        value: impl Into<String>,
    ) -> MarkerEnvironment {
        Arc::make_mut(&mut self.inner).platform_python_implementation = value.into();
        self
    }

    /// Set the release for this environment's platform.
    ///
    /// See also [`MarkerEnvironment::platform_release`].
    #[inline]
    #[must_use]
    pub fn with_platform_release(mut self, value: impl Into<String>) -> MarkerEnvironment {
        Arc::make_mut(&mut self.inner).platform_release = value.into();
        self
    }

    /// Set the system for this environment's platform.
    ///
    /// See also [`MarkerEnvironment::platform_system`].
    #[inline]
    #[must_use]
    pub fn with_platform_system(mut self, value: impl Into<String>) -> MarkerEnvironment {
        Arc::make_mut(&mut self.inner).platform_system = value.into();
        self
    }

    /// Set the version for this environment's platform.
    ///
    /// See also [`MarkerEnvironment::platform_version`].
    #[inline]
    #[must_use]
    pub fn with_platform_version(mut self, value: impl Into<String>) -> MarkerEnvironment {
        Arc::make_mut(&mut self.inner).platform_version = value.into();
        self
    }

    /// Set the full version of Python for this environment.
    ///
    /// See also [`MarkerEnvironment::python_full_version`].
    #[inline]
    #[must_use]
    pub fn with_python_full_version(
        mut self,
        value: impl Into<StringVersion>,
    ) -> MarkerEnvironment {
        Arc::make_mut(&mut self.inner).python_full_version = value.into();
        self
    }

    /// Set the version of Python for this environment.
    ///
    /// See also [`MarkerEnvironment::python_full_version`].
    #[inline]
    #[must_use]
    pub fn with_python_version(mut self, value: impl Into<StringVersion>) -> MarkerEnvironment {
        Arc::make_mut(&mut self.inner).python_version = value.into();
        self
    }

    /// Set the name of the system platform for this environment.
    ///
    /// See also [`MarkerEnvironment::sys_platform`].
    #[inline]
    #[must_use]
    pub fn with_sys_platform(mut self, value: impl Into<String>) -> MarkerEnvironment {
        Arc::make_mut(&mut self.inner).sys_platform = value.into();
        self
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl MarkerEnvironment {
    /// Construct your own marker environment
    #[new]
    #[pyo3(signature = (*,
        implementation_name,
        implementation_version,
        os_name,
        platform_machine,
        platform_python_implementation,
        platform_release,
        platform_system,
        platform_version,
        python_full_version,
        python_version,
        sys_platform
    ))]
    fn py_new(
        implementation_name: &str,
        implementation_version: &str,
        os_name: &str,
        platform_machine: &str,
        platform_python_implementation: &str,
        platform_release: &str,
        platform_system: &str,
        platform_version: &str,
        python_full_version: &str,
        python_version: &str,
        sys_platform: &str,
    ) -> PyResult<Self> {
        let implementation_version =
            StringVersion::from_str(implementation_version).map_err(|err| {
                PyValueError::new_err(format!(
                    "implementation_version is not a valid PEP440 version: {err}"
                ))
            })?;
        let python_full_version = StringVersion::from_str(python_full_version).map_err(|err| {
            PyValueError::new_err(format!(
                "python_full_version is not a valid PEP440 version: {err}"
            ))
        })?;
        let python_version = StringVersion::from_str(python_version).map_err(|err| {
            PyValueError::new_err(format!(
                "python_version is not a valid PEP440 version: {err}"
            ))
        })?;
        Ok(Self {
            inner: Arc::new(MarkerEnvironmentInner {
                implementation_name: implementation_name.to_string(),
                implementation_version,
                os_name: os_name.to_string(),
                platform_machine: platform_machine.to_string(),
                platform_python_implementation: platform_python_implementation.to_string(),
                platform_release: platform_release.to_string(),
                platform_system: platform_system.to_string(),
                platform_version: platform_version.to_string(),
                python_full_version,
                python_version,
                sys_platform: sys_platform.to_string(),
            }),
        })
    }

    /// Query the current python interpreter to get the correct marker value
    #[staticmethod]
    fn current(py: Python<'_>) -> PyResult<Self> {
        let os = py.import_bound("os")?;
        let platform = py.import_bound("platform")?;
        let sys = py.import_bound("sys")?;
        let python_version_tuple: (String, String, String) = platform
            .getattr("python_version_tuple")?
            .call0()?
            .extract()?;

        // See pseudocode at
        // https://packaging.python.org/en/latest/specifications/dependency-specifiers/#environment-markers
        let name = sys.getattr("implementation")?.getattr("name")?.extract()?;
        let info = sys.getattr("implementation")?.getattr("version")?;
        let kind = info.getattr("releaselevel")?.extract::<String>()?;
        let implementation_version: String = format!(
            "{}.{}.{}{}",
            info.getattr("major")?.extract::<usize>()?,
            info.getattr("minor")?.extract::<usize>()?,
            info.getattr("micro")?.extract::<usize>()?,
            if kind == "final" {
                String::new()
            } else {
                format!("{}{}", kind, info.getattr("serial")?.extract::<usize>()?)
            }
        );
        let python_full_version: String = platform.getattr("python_version")?.call0()?.extract()?;
        let python_version = format!("{}.{}", python_version_tuple.0, python_version_tuple.1);

        // This is not written down in PEP 508, but it's the only reasonable assumption to make
        let implementation_version =
            StringVersion::from_str(&implementation_version).map_err(|err| {
                PyValueError::new_err(format!(
                    "Broken python implementation, implementation_version is not a valid PEP440 version: {err}"
                ))
            })?;
        let python_full_version = StringVersion::from_str(&python_full_version).map_err(|err| {
            PyValueError::new_err(format!(
                "Broken python implementation, python_full_version is not a valid PEP440 version: {err}"
            ))
        })?;
        let python_version = StringVersion::from_str(&python_version).map_err(|err| {
            PyValueError::new_err(format!(
                "Broken python implementation, python_version is not a valid PEP440 version: {err}"
            ))
        })?;
        Ok(Self {
            inner: Arc::new(MarkerEnvironmentInner {
                implementation_name: name,
                implementation_version,
                os_name: os.getattr("name")?.extract()?,
                platform_machine: platform.getattr("machine")?.call0()?.extract()?,
                platform_python_implementation: platform
                    .getattr("python_implementation")?
                    .call0()?
                    .extract()?,
                platform_release: platform.getattr("release")?.call0()?.extract()?,
                platform_system: platform.getattr("system")?.call0()?.extract()?,
                platform_version: platform.getattr("version")?.call0()?.extract()?,
                python_full_version,
                python_version,
                sys_platform: sys.getattr("platform")?.extract()?,
            }),
        })
    }

    /// Returns the name of the Python implementation for this environment.
    #[getter]
    pub fn py_implementation_name(&self) -> String {
        self.implementation_name().to_string()
    }

    /// Returns the Python implementation version for this environment.
    #[getter]
    pub fn py_implementation_version(&self) -> StringVersion {
        self.implementation_version().clone()
    }

    /// Returns the name of the operating system for this environment.
    #[getter]
    pub fn py_os_name(&self) -> String {
        self.os_name().to_string()
    }

    /// Returns the name of the machine for this environment's platform.
    #[getter]
    pub fn py_platform_machine(&self) -> String {
        self.platform_machine().to_string()
    }

    /// Returns the name of the Python implementation for this environment's
    /// platform.
    #[getter]
    pub fn py_platform_python_implementation(&self) -> String {
        self.platform_python_implementation().to_string()
    }

    /// Returns the release for this environment's platform.
    #[getter]
    pub fn py_platform_release(&self) -> String {
        self.platform_release().to_string()
    }

    /// Returns the system for this environment's platform.
    #[getter]
    pub fn py_platform_system(&self) -> String {
        self.platform_system().to_string()
    }

    /// Returns the version for this environment's platform.
    #[getter]
    pub fn py_platform_version(&self) -> String {
        self.platform_version().to_string()
    }

    /// Returns the full version of Python for this environment.
    #[getter]
    pub fn py_python_full_version(&self) -> StringVersion {
        self.python_full_version().clone()
    }

    /// Returns the version of Python for this environment.
    #[getter]
    pub fn py_python_version(&self) -> StringVersion {
        self.python_version().clone()
    }

    /// Returns the name of the system platform for this environment.
    #[getter]
    pub fn py_sys_platform(&self) -> String {
        self.sys_platform().to_string()
    }
}

/// A builder for constructing a marker environment.
///
/// A value of this type can be fallibly converted to a full
/// [`MarkerEnvironment`] via [`MarkerEnvironment::try_from`]. This can fail when
/// the version strings given aren't valid.
///
/// The main utility of this type is for constructing dummy or test environment
/// values.
#[allow(missing_docs)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MarkerEnvironmentBuilder<'a> {
    pub implementation_name: &'a str,
    pub implementation_version: &'a str,
    pub os_name: &'a str,
    pub platform_machine: &'a str,
    pub platform_python_implementation: &'a str,
    pub platform_release: &'a str,
    pub platform_system: &'a str,
    pub platform_version: &'a str,
    pub python_full_version: &'a str,
    pub python_version: &'a str,
    pub sys_platform: &'a str,
}

impl<'a> TryFrom<MarkerEnvironmentBuilder<'a>> for MarkerEnvironment {
    type Error = VersionParseError;

    fn try_from(builder: MarkerEnvironmentBuilder<'a>) -> Result<Self, Self::Error> {
        Ok(MarkerEnvironment {
            inner: Arc::new(MarkerEnvironmentInner {
                implementation_name: builder.implementation_name.to_string(),
                implementation_version: builder.implementation_version.parse()?,
                os_name: builder.os_name.to_string(),
                platform_machine: builder.platform_machine.to_string(),
                platform_python_implementation: builder.platform_python_implementation.to_string(),
                platform_release: builder.platform_release.to_string(),
                platform_system: builder.platform_system.to_string(),
                platform_version: builder.platform_version.to_string(),
                python_full_version: builder.python_full_version.parse()?,
                python_version: builder.python_version.parse()?,
                sys_platform: builder.sys_platform.to_string(),
            }),
        })
    }
}
