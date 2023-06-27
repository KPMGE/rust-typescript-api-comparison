import express from 'express'
import { routes } from './config'

const app = express()
app.use(express.json())
app.use(routes)

export { app }
