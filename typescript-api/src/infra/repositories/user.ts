import { PrismaClient } from "@prisma/client";
import { 
  CheckUserRepository,
  CreateUserRepository, 
  DeleteUserRepository, 
  ListUserRepository,
  UpdateUserRepository
} from "../../data/repositories";
import { User } from "../../domain/entities";

const prisma = new PrismaClient()

export class UserRepository implements 
  CreateUserRepository, 
  CheckUserRepository, 
  ListUserRepository,
  UpdateUserRepository,
  DeleteUserRepository {

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

  async list(): Promise<User[]> {
    return await prisma.user.findMany()
  }

  async update(userId: number, newUser: Omit<User, 'id'>): Promise<void> {
    await prisma.user.update({
      where: {
        id: userId
      },
      data: {
        name: newUser.name,
        email: newUser.email,
      }
    })
  }

  async delete(userId: number): Promise<void> {
    await prisma.user.delete({
      where: {
        id: userId
      }
    })
  }
}

