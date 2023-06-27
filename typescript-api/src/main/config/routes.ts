import { Request, Response } from 'express'
import { Router } from "express";
import { adaptRoute } from '../adapters';
import { makeCreateUserController, makeListUserController } from '../factories';

const routes = Router()

routes.get('/health_check', (_req: Request, res: Response) => {
  res.send()
})

routes.get('/users', adaptRoute(makeListUserController()))
routes.post('/users', adaptRoute(makeCreateUserController()))

export { routes }
