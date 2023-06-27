import { CreateUserService } from "../../data/services"
import { UserRepository } from "../../infra/repositories"
import { CreateUserController } from "../../presentation/controllers"

export const makeCreateUserController = () => {
  const userRepo = new UserRepository()
  const service = new CreateUserService(userRepo, userRepo)
  return new CreateUserController(service)
} 
