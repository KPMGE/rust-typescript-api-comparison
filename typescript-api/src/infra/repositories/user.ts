import { PrismaClient } from "@prisma/client";
import { 
  CheckUserRepository,
  CreateUserRepository 
} from "../../data/repositories";
import { User } from "../../domain/entities";

const prisma = new PrismaClient()

export class UserRepository implements CreateUserRepository, CheckUserRepository {
  async create(user: User): Promise<void> {
    await prisma.user.create({
      data: {
        name: user.name, 
        password: user.password, 
        email: user.email
      }
    })
  }

  async exists(user: User): Promise<boolean> {
    const foundUser = await prisma.user.findFirst({
      where: {
        name: user.name
      }
    })

    return !!foundUser 
  }
}

