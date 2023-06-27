import { ListTodoService } from "../../data/services"
import { TodoRepository } from "../../infra/repositories"
import { ListTodoController } from "../../presentation/controllers/list-todo"

export const makeListTodoController = () => {
  const repo = new TodoRepository()
  const service = new ListTodoService(repo)
  return new ListTodoController(service)
}
