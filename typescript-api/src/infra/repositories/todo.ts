import { PrismaClient } from "@prisma/client";
import { CreateTodoRepository } from "../../data/repositories";
import { Todo } from "../../domain/entities";

const prisma = new PrismaClient()

export class TodoRepository implements CreateTodoRepository {
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
}
