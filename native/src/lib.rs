use neon::prelude::*;
use rusqlite::{params, Connection, Result};


struct QueryResult {
    key: String,
    value: String,
}

pub struct QuickDB {
    pub table: String,
    conn: Connection,
}

impl QuickDB {

    pub fn test(&self, key: &str) {
        println!("Table: {}, key: {}", self.table, key);
    }

    pub fn init(&self) -> Result<usize> {
        self.conn.execute("PRAGMA synchronous = OFF", params![])?;
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS data (key TEXT PRIMARY KEY, value TEXT)",
            params![]
        )
    }

    pub fn set(&self, key: String, value: String) -> Result<usize> {
        self.conn.execute("INSERT INTO data (key,value) VALUES (?1,?2)", params![key, value])
    }

    pub fn delete(&self, key: String) -> Result<usize> {
        self.conn.execute("DELETE FROM data WHERE key = ?1", params![key])
    }
}

pub struct TableFactory {
}

impl TableFactory {
    pub fn new() -> Self {
        TableFactory {}
    }

    pub fn create(&self, table: String) -> Result<QuickDB> {
        let conn = Connection::open("./data/data.db")?;

        Ok(QuickDB {
            table: table,
            conn: conn,
        })
    }
}

declare_types! {
    pub class JsQuickDB for QuickDB {
        init(mut c) {
            let factory = c.argument::<JsTableFactory>(0)?;
            let table = c.argument::<JsString>(1)?.value();

            let guard = c.lock();
            let factory = factory.borrow(&guard);

            match factory.create(table) {
                Ok(created) => Ok(created),
                _ => panic!("Something weird happened."),
            }
        }

        method test(mut c) {
            let key = c.argument::<JsString>(0)?;
            let this = c.this();
            let guard = c.lock();

            this.borrow(&guard).test(&key.value());

            Ok(c.undefined().upcast())
        }

        method init(mut c) {
            let this = c.this();
            let guard = c.lock();

            match this.borrow(&guard).init() {
                Ok(result) => result,
                Err(e) => panic!("{}", e),
            };

            Ok(c.undefined().upcast())
        }

        method set(mut c) {
            let key = c.argument::<JsString>(0)?.value();
            let value = c.argument::<JsString>(1)?.value();
            let this = c.this();
            let guard = c.lock();

            match this.borrow(&guard).set(key, value) {
                Ok(result) => result,
                Err(e) => panic!("{}", e),
            };

            Ok(c.undefined().upcast())
        }

        method delete(mut c) {
            let key = c.argument::<JsString>(0)?.value();
            let this = c.this();
            let guard = c.lock();

            match this.borrow(&guard).delete(key) {
                Ok(result) => result,
                Err(e) => panic!("{}", e),
            };

            Ok(c.undefined().upcast())
        }

    }

    pub class JsTableFactory for TableFactory {
        init(_) {
            Ok(TableFactory::new())
        }

        method create(mut c) {
            let table = c.argument::<JsValue>(0)?;
            let this = c.this().upcast();

            Ok(JsQuickDB::new(&mut c, vec![this, table])?.upcast())
        }
    }
}


register_module!(mut cx, {
    cx.export_class::<JsTableFactory>("TableFactory")?;

    Ok(())
});