mod kvstore;
use crate::kvstore::KVStore;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyList};
use pyo3::{Bound, Python, PyResult};


#[pymodule]
fn wind_kvcore(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<_WindKVCore>()?;
    Ok(())
}


#[pyclass]
struct _WindKVCore {
    // 使用Option包装，以便在close时可以take所有权
    inner: Option<KVStore>,
}


#[pymethods]
impl _WindKVCore {
    /**
    * wind_kvcore 模块提供了一个高效的键值存储引擎，用于持久化存储键值对数据。
    *
    * The wind_kvcore module provides an efficient key-value storage engine for persistently storing key-value pairs.
    *
    * 键值存储核心类，提供数据的存储、读取、删除和管理功能。
    *
    * The core class for key-value storage, providing functionality for storing, reading,
    * deleting and managing data.
    *
    *
    */
    #[new]
    fn new(path: &str, db_identifier: Option<&str>) -> PyResult<Self> {
        let store = KVStore::open(path, db_identifier)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
        Ok(Self { inner: Some(store) })
    }

    /// 获取键值 - 修复生命周期问题
    fn get<'a>(&'a mut self, py: Python<'a>, key: &[u8]) -> PyResult<Option<Bound<'a, PyBytes>>> {
        // 检查KVStore是否已关闭
        let store = self.inner.as_mut()
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("KVStore already closed"))?;
            
        let result = store.get(key)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
        
        Ok(match result {
            Some(bytes) => Some(PyBytes::new(py, &bytes).into()),
            None => None,
        })
    }

    /// Returns:
    /// List[Optional[Dict[str, str]]]
    /// 
    fn get_all<'a>(&'a mut self, py: Python<'a>) -> PyResult<Bound<'a, PyList>> {
        let store = self.inner.as_mut()
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("KVStore already closed"))?;

        let result = store.get_all()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;

        let py_items: Vec<Bound<'a, PyDict>> = result
            .into_iter()
            .map(|(key_bytes, value_bytes)| {
                let dict = PyDict::new(py);

                // 转换为字符串（如果数据是有效的 UTF-8）
                let key_str = std::str::from_utf8(&key_bytes)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyUnicodeDecodeError, _>(format!("Invalid UTF-8 in key: {}", e)))?;
                let value_str = std::str::from_utf8(&value_bytes)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyUnicodeDecodeError, _>(format!("Invalid UTF-8 in value: {}", e)))?;

                dict.set_item(key_str, value_str)?;
                Ok(dict)
            })
            .collect::<PyResult<Vec<_>>>()?;

        Ok(PyList::new(py, py_items)?)
    }

    /// 设置键值
    fn put(&mut self, key: &[u8], value: &[u8]) -> PyResult<()> {
        let store = self.inner.as_mut()
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("KVStore already closed"))?;
            
        store.put(key, value)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
        Ok(())
    }

    /// 删除键
    fn delete(&mut self, key: &[u8]) -> PyResult<()> {
        let store = self.inner.as_mut()
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("KVStore already closed"))?;
            
        store.delete(key)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
        Ok(())
    }

    /// 设置数据库标识
    fn set_identifier(&mut self, identifier: &str) -> PyResult<()> {
        let store = self.inner.as_mut()
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("KVStore already closed"))?;
            
        store.set_identifier(identifier)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
        Ok(())
    }

    /// 压缩数据库
    fn compact(&mut self) -> PyResult<()> {
        let store = self.inner.as_mut()
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("KVStore already closed"))?;
            
        store.compact()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
        Ok(())
    }

    /// 关闭数据库 - 修复所有权问题
    fn close(&mut self) -> PyResult<()> {
        // 取出inner的所有权
        if let Some(store) = self.inner.take() {
            store.close()
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
        }
        Ok(())
    }

    /// 获取数据库标识
    fn get_identifier(&self) -> PyResult<&str> {
        let store = self.inner.as_ref()
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("KVStore already closed"))?;
            
        Ok(store.get_identifier())
    }
}
