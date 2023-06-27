import { Request, Response } from 'express'
import { Router } from "express";
import { adaptRoute } from '../adapters';
import { 
  makeCreateTodoRepository,
  makeCreateUserController, 
  makeDeleteTodoController, 
  makeDeleteUserController, 
  makeListTodoController, 
  makeListUserController, 
  makeUpdateTodoController, 
  makeUpdateUserController 
} from '../factories';

const routes = Router()

routes.get('/health_check', (_req: Request, res: Response) => {
  res.send()
})

routes.post('/users', adaptRoute(makeCreateUserController()))
routes.get('/users', adaptRoute(makeListUserController()))
routes.put('/users/:userId', adaptRoute(makeUpdateUserController()))
routes.delete('/users/:userId', adaptRoute(makeDeleteUserController()))

routes.post('/todos/:userId', adaptRoute(makeCreateTodoRepository()))
routes.get('/todos/:userId', adaptRoute(makeListTodoController()))
routes.put('/todos/:tagId', adaptRoute(makeUpdateTodoController()))
routes.delete('/todos/:tagId', adaptRoute(makeDeleteTodoController()))

export { routes }
