const { TableFactory } = require('../native/index.node');

const factory = new TableFactory();
const db = factory.create('test', './data/data.sqlite');


console.time('init')
db.init()
console.timeEnd('init')

console.time('loop_set_native')

for (let i = 0; i < 10000; i++) {
    db.set(i.toString(), i.toString());
}

console.timeEnd('loop_set_native');

console.time('loop_get_native');
for (let i = 0; i < 10000; i++) {
    db.get(i.toString());
}
console.timeEnd('loop_get_native');


console.time('loop_delete_native');
for (let i = 0; i < 10000; i++) {
    db.delete(i.toString());
}
console.timeEnd('loop_delete_native');
