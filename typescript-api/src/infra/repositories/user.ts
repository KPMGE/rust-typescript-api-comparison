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
    await prisma.$transaction([
      prisma.user.create({
        data: {
          name: user.name,
          password: user.password,
          email: user.email
        }
      })
    ])
  }

  async exists(user: User): Promise<boolean> {
    const [foundUser] = await prisma.$transaction([
      prisma.user.findFirst({
        where: {
          name: user.name
        }
      })
    ])

    return !!foundUser
  }

  async list(): Promise<User[]> {
    const [users] = await prisma.$transaction([
      prisma.user.findMany()
    ])

    return users
  }

  async update(userId: number, newUser: Omit<User, 'id'>): Promise<void> {
    await prisma.$transaction([
      prisma.user.update({
        where: {
          id: userId
        },
        data: {
          name: newUser.name,
          email: newUser.email,
        }
      })
    ])
  }

  async delete(userId: number): Promise<void> {
    prisma.$transaction([
      prisma.user.delete({
        where: {
          id: userId
        }
      })
    ])
  }
}

