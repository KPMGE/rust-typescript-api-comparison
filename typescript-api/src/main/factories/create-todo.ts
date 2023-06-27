import { CreateTodoService } from "../../data/services"
import { TodoRepository } from "../../infra/repositories"
import { CreateTodoController } from "../../presentation/controllers/create-todo"

export const makeCreateTodoRepository = () => {
  const repo = new TodoRepository()
  const service = new CreateTodoService(repo)
  return new CreateTodoController(service)
}
