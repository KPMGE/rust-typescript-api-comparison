import { UpdateTodoService } from "../../data/services"
import { TodoRepository } from "../../infra/repositories"
import { UpdateTodoController } from "../../presentation/controllers"

export const makeUpdateTodoController = () => {
  const repo = new TodoRepository()
  const service = new UpdateTodoService(repo)
  return new UpdateTodoController(service)
}
