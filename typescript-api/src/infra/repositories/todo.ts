import { PrismaClient } from "@prisma/client";
import { CreateTodoRepository, DeleteTodoRepository, ListTodoRepository, UpdateTodoRepository } from "../../data/repositories";
import { Todo } from "../../domain/entities";

const prisma = new PrismaClient()

export class TodoRepository implements
  CreateTodoRepository,
  ListTodoRepository,
  UpdateTodoRepository,
  DeleteTodoRepository {

  async create(userId: number, todo: Omit<Todo, "id">): Promise<void> {
    await prisma.$transaction([
      prisma.todo.create({
        data: {
          title: todo.title,
          description: todo.description,
          completed: false,
          userId: userId
        }
      })
    ])
  }

  async list(userId: number): Promise<Todo[]> {
    const [todos] = await prisma.$transaction([
      prisma.todo.findMany({
        where: {
          userId
        }
      })
    ])

    return todos
  }

  async update(todo: Todo): Promise<void> {
    await prisma.$transaction([
      prisma.todo.update({
        where: { id: todo.id },
        data: {
          completed: todo.completed,
          description: todo.description,
          title: todo.title
        }
      })
    ])
  }

  async delete(todoId: number): Promise<void> {
    await prisma.$transaction([
      prisma.todo.delete({
        where: {
          id: todoId
        }
      })
    ])
  }
}
