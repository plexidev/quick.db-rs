const { TableFactory } = require('../native/index.node');
const fs = require('fs');

if (!fs.existsSync('./data')) {
	fs.mkdirSync('./data');
}

const factory = new TableFactory();
const db = factory.create('test', './data/data.sqlite');

db.init();


function set(key, value) {
	if (typeof key !== 'string') throw new Error(`Unable to set a key with the type of "${typeof key}", keys must be a string!`);
	if (typeof value === 'object') {
		return db.set(key, JSON.stringify(value));
	} else return db.set(key, value);
}

function get(key) {
	if (typeof key !== 'string') throw new Error(`Unable to set a key with the type of "${typeof key}", keys must be a string!`);
	const value = db.get(key);
	return value !== "" ? value : undefined;
}

function remove(key) {
	if (typeof key !== 'string') throw new Error(`Unable to set a key with the type of "${typeof key}", keys must be a string!`);
	return db.delete(key);
}

module.exports = {
	set,
	get,
	remove
}
