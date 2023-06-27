import { Todo } from "../../domain/entities";

export interface ListTodoRepository {
  list(userId: number): Promise<Todo[]>
}
