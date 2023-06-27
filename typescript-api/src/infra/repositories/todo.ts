import { PrismaClient } from "@prisma/client";
import { CreateTodoRepository, ListTodoRepository } from "../../data/repositories";
import { Todo } from "../../domain/entities";

const prisma = new PrismaClient()

export class TodoRepository implements CreateTodoRepository, ListTodoRepository {
  async create(userId: number, todo: Omit<Todo, "id">): Promise<void> {
    await prisma.todo.create({
      data: {
        title: todo.title,
        description: todo.description,
        completed: false,
        userId: userId
      }
    })
  }

  async list(userId: number): Promise<Todo[]> {
    return await prisma.todo.findMany({
      where: {
        userId
      }
    })
  }
}
