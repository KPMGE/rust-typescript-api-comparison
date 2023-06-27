import { Request, Response } from 'express'
import { Router } from "express";
import { adaptRoute } from '../adapters';
import { 
  makeCreateUserController, 
  makeDeleteUserController, 
  makeListUserController, 
  makeUpdateUserController 
} from '../factories';

const routes = Router()

routes.get('/health_check', (_req: Request, res: Response) => {
  res.send()
})

routes.get('/users', adaptRoute(makeListUserController()))
routes.post('/users', adaptRoute(makeCreateUserController()))
routes.put('/users/:userId', adaptRoute(makeUpdateUserController()))
routes.delete('/users/:userId', adaptRoute(makeDeleteUserController()))

export { routes }
