const { TableFactory } = require('../native/index.node');

const factory = new TableFactory();
const db = factory.create('test');

console.time('init')
db.init()
console.timeEnd('init')


console.time('set')

db.set('test', 'test');

console.timeEnd('set')


console.time('delete')

db.delete('test');

console.timeEnd('delete')