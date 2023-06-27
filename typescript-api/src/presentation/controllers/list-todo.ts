import { Request, Response } from 'express'
import { ListTodoService } from "../../data/services";
import httpStatus from 'http-status-codes'

export class ListTodoController {
  constructor(
    private readonly service: ListTodoService
  ) {}

  async handle(req: Request, res: Response) {
    const userId = Number(req.params.userId)

    try {
      const todos = await this.service.list(userId)
      return res.status(httpStatus.OK).json(todos)
    } catch (error) {
      if (process.env.SHOW_ERRORS) {
        console.error('ERROR: ', error)
      }

      return res.status(httpStatus.INTERNAL_SERVER_ERROR).json({
        message: 'internal server error'
      })
    }
  }
}
