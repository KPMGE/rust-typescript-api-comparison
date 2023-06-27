import { ListUserService } from "../../data/services"
import { UserRepository } from "../../infra/repositories"
import { ListUserController } from "../../presentation/controllers"

export const makeListUserController = () => {
  const repo = new UserRepository()
  const service = new ListUserService(repo)
  return new ListUserController(service)
}
