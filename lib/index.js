const { TableFactory } = require('../native/index.node');
const fs = require('fs');

if (!fs.existsSync('./data')) {
	fs.mkdirSync('./data');
}

const factory = new TableFactory();
const db = factory.create('test', './data/data.sqlite');

db.init();


function set(key, value) {
	return db.set(key, value);
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

function getAll() {
	return db.get_all().map(r => {
		const key = r.key;
		let value;

		try {
			value = JSON.parse(r.value);
		} catch (e) {
			value = r.value;
		}

		return { key, value }
	});
}

module.exports = {
	set,
	get,
	remove,
	getAll
}
