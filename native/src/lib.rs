use neon::prelude::*;
use sqlite::*;

pub struct QuickDB {
    pub table: String,
    conn: Connection,
}

// pub struct QuickDBRow {
//     key: String,
//     value: String,
// }

impl QuickDB {

    pub fn test(&self, key: &str) {
        println!("Table: {}, key: {}", self.table, key);
    }

    pub fn init(&self) -> () {
        self.conn.execute("PRAGMA synchronous = OFF;").unwrap();
        self.conn.execute(format!("CREATE TABLE IF NOT EXISTS {} (key TEXT PRIMARY KEY, value TEXT);", self.table)).unwrap()
    }

    pub fn set(&self, key: String, value: String) -> () {
        self.conn.execute(format!("INSERT INTO {} VALUES ('{}', '{}');", self.table, key, value)).unwrap()
    }

    pub fn delete(&self, key: String) -> usize {
        self.conn.prepare(format!("DELETE FROM {} WHERE key = '{}';", self.table, key)).unwrap().count()
    }

    pub fn get(&self, key: String) -> String {
        let mut stmt = self.conn.prepare(format!("SELECT * FROM {} WHERE key = '{}';", self.table, key)).unwrap();

        let mut val: String = String::new();
        while let State::Row = stmt.next().unwrap() {
            val = stmt.read::<String>(1).unwrap();
        };

        val
    }
}

pub struct TableFactory {}

impl TableFactory {
    pub fn new() -> Self {
        TableFactory {}
    }

    pub fn create(&self, table: String, path: String) -> QuickDB {
        let conn = Connection::open(path).unwrap();

        QuickDB {
            table: table,
            conn: conn,
        }
    }
}

declare_types! {
    pub class JsQuickDB for QuickDB {
        init(mut c) {
            let factory = c.argument::<JsTableFactory>(0)?;
            let table = c.argument::<JsString>(1)?.value();
            let path = c.argument::<JsString>(2)?.value();

            let guard = c.lock();
            let factory = factory.borrow(&guard);

            Ok(factory.create(table, path))
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

            this.borrow(&guard).init();

            Ok(c.undefined().upcast())
        }

        method set(mut c) {
            let key = c.argument::<JsString>(0)?.value();
            let value = c.argument::<JsString>(1)?.value();
            let this = c.this();
            let guard = c.lock();

            this.borrow(&guard).set(key, value);

            Ok(c.undefined().upcast())
        }

        method delete(mut c) {
            let key = c.argument::<JsString>(0)?.value();
            let this = c.this();
            let guard = c.lock();

            this.borrow(&guard).delete(key);

            Ok(c.undefined().upcast())
        }

        method get(mut c) {
            let key = c.argument::<JsString>(0)?.value();
            let this = c.this();
            let guard = c.lock();

            let value = this.borrow(&guard).get(key);

            Ok(c.string(&value).upcast())
        }

    }

    pub class JsTableFactory for TableFactory {
        init(_) {
            Ok(TableFactory::new())
        }

        method create(mut c) {
            let table = c.argument::<JsValue>(0)?;
            let path = c.argument::<JsValue>(1)?;
            let this = c.this().upcast();

            Ok(JsQuickDB::new(&mut c, vec![this, table, path])?.upcast())
        }
    }
}


register_module!(mut cx, {
    cx.export_class::<JsTableFactory>("TableFactory")?;

    Ok(())
});

