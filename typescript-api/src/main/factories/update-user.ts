import { UpdateUserService } from "../../data/services"
import { UserRepository } from "../../infra/repositories"
import { UpdateUserController } from "../../presentation/controllers"

export const makeUpdateUserController = () => {
  const repo = new UserRepository()
  const service = new UpdateUserService(repo)
  return new UpdateUserController(service)
}
