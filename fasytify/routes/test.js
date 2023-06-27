'use strict'


const { readFile } = require('node:fs/promises');
const { join } = require('path');

module.exports = async function (fastify, opts) {
  fastify.get('/test', async function (request, reply) {

    const contents = await readFile(join(__dirname, '..', 'example_849K.json'), { encoding: 'utf8' });

    return contents;
  })
}
