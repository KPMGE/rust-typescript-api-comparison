import { Request, Response } from 'express'
import express from 'express'

const app = express()

app.get('/health_check', (_req: Request, res: Response) => {
  res.send()
})

const port = 3333
app.listen(port, () => console.log(`Listening on: http://127.0.0.1:${port}`))
