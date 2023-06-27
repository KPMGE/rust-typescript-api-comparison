import { Todo } from '../../domain/entities'

export interface UpdateTodoRepository {
  update(todo: Todo): Promise<void>
}
