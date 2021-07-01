import express from 'express'
let app = express()

app.get('/asdf', (_, res) => res.send('adf'))

app.listen(8000, () => console.log('started'))
