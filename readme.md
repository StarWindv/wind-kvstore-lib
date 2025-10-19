# Wind-KVStore Python SDK

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Python](https://img.shields.io/badge/python-3.10%2B-blue)
![Rust](https://img.shields.io/badge/rust-2024%20edition-orange)

A Python wrapper for a high-performance Rust-based key-value storage engine, providing a stable and reliable data persistence solution.

## Project Structure

```
wind-kvstore-lib/
├── Cargo.toml          # Rust project configuration
├── pyproject.toml      # Python packaging configuration
├── src/
│   ├── lib.rs          # PyO3 module entry point
│   └── kvstore.rs      # Core storage engine implementation
└── wind_kvcore/
    ├── __init__.py     # Package exports
    ├── WindKVCore.py   # Python wrapper class
    └── wind_kvcore.pyi # Type hint file
```

## Requirements

### System Requirements
- **Operating Systems**: Windows, Linux, macOS
- **Python**: 3.10 or higher
- **Rust Toolchain**: Required for compiling from source

### Required Tools
1. **Python 3.10+**
2. **Rust Toolchain**
   ```bash
   # Install Rust
   sudo snap install rustup
   ```

3. **Maturin**
   ```bash
   pip install maturin
   ```

## Installation

### Building from Source

1. Clone the project:
```bash
git clone https://github.com/starwindv/wind-kvstore-lib
cd wind-kvstore-lib
```

2. Build and install using maturin:
```bash
maturin build
pip install target/wheels/wind_kvcore-*.whl
```

## Usage

### Basic Operations

```python
from wind_kvcore import WindKVCore

# Open database (creates if it doesn't exist)
with WindKVCore("./mydatabase.db") as db:
    # Store data
    db.put(b"key1", b"value1")
    db.put(b"key2", b"value2")
    
    # Read data
    value = db.get(b"key1")
    print(f"key1: {value}")  # Output: b'value1'
    
    # Delete data
    db.delete(b"key2")
    
    # Get all key-value pairs
    all_data = db.get_all()
    for item in all_data:
        print(f"{item['key']}: {item['value']}")
```

### Database Identifier Management

```python
# Specify identifier during creation
db = WindKVCore("./data.db", "my_database")

# Or set identifier later
db.set_identifier("new_identifier")
current_id = db.get_identifier()
print(f"Current database identifier: {current_id}")
```

### Performance Optimization

```python
# Use context manager to ensure proper resource cleanup
with WindKVCore("./data.db") as db:
    # Perform operations...
    pass
```

## API Reference

### WindKVCore Class

#### Initialization
```python
WindKVCore(path: str, db_identifier: Optional[str] = None)
```

#### Main Methods
- `get(key: bytes) -> Optional[bytes]` - Get value by key
- `put(key: bytes, value: bytes) -> None` - Store or update key-value pair
- `delete(key: bytes) -> None` - Delete key-value pair
- `get_all() -> List[Dict[str, str]]` - Get all key-value pairs
- `compact() -> None` - Compact database to optimize performance
- `set_identifier(identifier: str) -> None` - Set database identifier
- `get_identifier() -> str` - Get current database identifier
- `close() -> None` - Close database connection

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Original Project

- Project Homepage: [GitHub Repository](https://github.com/StarWindv/Wind-KVStore)
- Author: StarWindv
