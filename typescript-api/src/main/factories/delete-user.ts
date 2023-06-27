import { DeleteUserService } from "../../data/services"
import { UserRepository } from "../../infra/repositories"
import { DeleteUserController } from "../../presentation/controllers"

export const makeDeleteUserController = () => {
  const repo = new UserRepository()
  const service = new DeleteUserService(repo)
  return new DeleteUserController(service)
}
