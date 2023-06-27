import { DeleteTodoService } from "../../data/services";
import { Request, Response } from 'express'
import httpStatus from 'http-status-codes'

export class DeleteTodoController {
  constructor(
    private readonly service: DeleteTodoService
  ) {}

  async handle(req: Request, res: Response) {
    const tagId = Number(req.params.tagId)

    try {
      await this.service.delete(tagId)
      return res.status(httpStatus.OK).send()
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
