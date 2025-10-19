# Wind-KVStore Python SDK

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Python](https://img.shields.io/badge/python-3.10%2B-blue)
![Rust](https://img.shields.io/badge/rust-2024%20edition-orange)

基于 Rust 高性能键值存储引擎的 Python 封装，提供稳定可靠的数据持久化解决方案。

## 项目结构

```
wind-kvstore-lib/
├── Cargo.toml          # Rust 项目配置
├── pyproject.toml      # Python 打包配置
├── src/
│   ├── lib.rs          # PyO3 模块入口
│   └── kvstore.rs      # 核心存储引擎实现
└── wind_kvcore/
    ├── __init__.py     # 包导出
    ├── WindKVCore.py   # Python 包装类
    └── wind_kvcore.pyi # 类型提示文件
```

## 环境要求

### 系统要求
- **操作系统**: Windows, Linux, macOS
- **Python**: 3.10 或更高版本
- **Rust 工具链**: 用于从源码编译

### 必需工具
1. **Python 3.10+**
2. **Rust 工具链**
   ```bash
   # 安装 Rust
   sudo snap install rustup
   ```

3. **Maturin**
   ```bash
   pip install maturin
   ```

## 安装方式

### 从源码构建

1. 克隆项目：
```bash
git clone https://github.com/starwindv/wind-kvstore-lib
cd wind-kvstore-lib
```

2. 使用 maturin 构建并安装：
```bash
maturin build
pip install target/wheels/wind_kvcore-*.whl
```


## 使用方法

### 基本操作

```python
from wind_kvcore import WindKVCore

# 打开数据库（如果不存在会自动创建）
with WindKVCore("./mydatabase.db") as db:
    # 存储数据
    db.put(b"key1", b"value1")
    db.put(b"key2", b"value2")
    
    # 读取数据
    value = db.get(b"key1")
    print(f"key1: {value}")  # 输出: b'value1'
    
    # 删除数据
    db.delete(b"key2")
    
    # 获取所有键值对
    all_data = db.get_all()
    for item in all_data:
        print(f"{item['key']}: {item['value']}")
```

### 数据库标识管理

```python
# 创建时指定标识
db = WindKVCore("./data.db", "my_database")

# 或后续设置标识
db.set_identifier("new_identifier")
current_id = db.get_identifier()
print(f"当前数据库标识: {current_id}")
```

### 性能优化

```python
# 使用上下文管理器确保资源正确释放
with WindKVCore("./data.db") as db:
    # 执行操作...
    pass
```

## API 参考

### WindKVCore 类

#### 初始化
```python
WindKVCore(path: str, db_identifier: Optional[str] = None)
```

#### 主要方法
- `get(key: bytes) -> Optional[bytes]` - 根据键获取值
- `put(key: bytes, value: bytes) -> None` - 存储或更新键值对
- `delete(key: bytes) -> None` - 删除键值对
- `get_all() -> List[Dict[str, str]]` - 获取所有键值对
- `compact() -> None` - 压缩数据库优化性能
- `set_identifier(identifier: str) -> None` - 设置数据库标识
- `get_identifier() -> str` - 获取当前数据库标识
- `close() -> None` - 关闭数据库连接


## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 原项目

- 项目主页: [GitHub Repository](https://github.com/StarWindv/Wind-KVStore)
- 作者: StarWindv(星灿长风v)
