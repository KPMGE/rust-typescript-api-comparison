import { Request, Response } from 'express'
import { UpdateUserService } from "../../data/services";
import { User } from '../../domain/entities';
import httpStatus from 'http-status-codes'
import validator from 'validator';

export class UpdateUserController {
  constructor(
    private readonly service: UpdateUserService
  ) { }

  async handle(req: Request, res: Response) {
    const userId: number = Number(req.params.userId)
    const newUser: Omit<User, 'id' | 'password'> = {
      email: req.body.email,
      name: req.body.name
    }

    try {
      if (!userId || !validator.isEmail(newUser.email) || !newUser.name) {
        return res.status(httpStatus.BAD_REQUEST).send()
      }

      await this.service.update(userId, newUser)

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
