import { DeleteTodoService } from "../../data/services"
import { TodoRepository } from "../../infra/repositories"
import { DeleteTodoController } from "../../presentation/controllers"

export const makeDeleteTodoController = () => {
  const repo = new TodoRepository()
  const service = new DeleteTodoService(repo)
  return new DeleteTodoController(service)
}
