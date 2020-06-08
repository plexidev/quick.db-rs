const { TableFactory } = require('../native/index.node');

const factory = new TableFactory();
const db = factory.create('test');

console.time('init')
db.init()
console.timeEnd('init')


console.time('set1k')

for (let i = 0; i < 1000; i++) {
	db.set(i.toString(), i.toString());
}

console.timeEnd('set1k')