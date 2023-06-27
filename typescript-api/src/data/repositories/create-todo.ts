import { Todo } from "../../domain/entities";

export interface CreateTodoRepository {
  create(userId: number, todo: Omit<Todo, 'id'>): Promise<void>
}
