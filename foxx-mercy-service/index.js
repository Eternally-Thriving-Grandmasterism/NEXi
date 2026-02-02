'use strict';
const createRouter = require('@arangodb/foxx/router');
const joi = require('joi');
const db = require('@arangodb').db;
const aql = require('aql');

const router = createRouter();
module.context.use(router);

// Valence threshold constant (eternal mercy gate)
const VALENCE_THRESHOLD = 0.9999999;

// POST /mercy/validate - Check valence mercy gate
router.post('/validate', (req, res) => {
  const { valence } = req.body;
  if (typeof valence !== 'number' || valence < VALENCE_THRESHOLD) {
    res.status(403).send({ error: 'Mercy shield: low valence rejected' });
    return;
  }
  res.send({ approved: true, message: 'Mercy-approved: eternal thriving' });
})
.body(joi.object({
  valence: joi.number().required()
}), 'Valence to check')
.response(['application/json'], 'Approval result')
.summary('Mercy valence gate check')
.description('Server-side mercy validation for operations');

// POST /mercy/insert-atom - Insert .metta atom if valence high
router.post('/insert-atom', (req, res) => {
  const { text, valence, context } = req.body;
  if (valence < VALENCE_THRESHOLD) {
    res.status(403).send({ error: 'Mercy shield: low valence persistence rejected' });
    return;
  }

  try {
    const result = db._executeTransaction({
      collections: { write: 'MettaAtoms' },
      action: function () {
        const db = require('@arangodb').db;
        return db.MettaAtoms.save({
          text,
          valence,
          timestamp: new Date().toISOString(),
          context: context || 'default'
        });
      }
    });

    res.send({ success: true, id: result._key, message: 'Persisted in mercy lattice' });
  } catch (e) {
    res.status(500).send({ error: 'Insert failed: ' + e.message });
  }
})
.body(joi.object({
  text: joi.string().required(),
  valence: joi.number().required(),
  context: joi.string().optional()
}), '.metta atom details')
.response(['application/json'], 'Insert result')
.summary('Mercy-gated atom insert')
.description('AQL insert with transaction + valence check');

// GET /mercy/high-valence - Query high-valence atoms
router.get('/high-valence', (req, res) => {
  const min = parseFloat(req.queryParams.min) || VALENCE_THRESHOLD;
  const cursor = db._query(aql`
    FOR atom IN MettaAtoms
    FILTER atom.valence >= ${min}
    RETURN { text: atom.text, valence: atom.valence, context: atom.context }
  `);

  res.send(cursor.toArray());
})
.queryParam('min', joi.number().default(VALENCE_THRESHOLD), 'Minimum valence')
.response(['application/json'], 'High-valence atoms')
.summary('Query high-valence mercy atoms')
.description('AQL seek for eternal thriving rules');
