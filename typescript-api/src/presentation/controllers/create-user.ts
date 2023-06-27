import { Request, Response } from 'express'
import { CreateUserService } from "../../data/services";
import { DuplicateUserNameError, InvalidEmailError, InvalidPasswordError, InvalidUserNameLenghtError } from '../../domain/errors';
import { User } from '@prisma/client';
import httpStatus from 'http-status-codes'

export class CreateUserController {
  constructor(private readonly service: CreateUserService) { }

  async handle(req: Request, res: Response) {
    const user: Omit<User, 'id'> = {
      name: req.body.name,
      email: req.body.email,
      password: req.body.password
    }

    console.log('user: ', user)

    try {
      await this.service.create(user)
      return res.status(httpStatus.CREATED).send()
    } catch (error) {
      if (process.env.SHOW_ERRORS) {
        console.error('ERROR: ', error)
      }

      if (
        error instanceof InvalidEmailError
        || error instanceof InvalidPasswordError
        || error instanceof InvalidUserNameLenghtError
        || error instanceof DuplicateUserNameError
      ) {
        return res.status(httpStatus.BAD_REQUEST).json({
          message: error.message
        })
      }

      return res.status(httpStatus.INTERNAL_SERVER_ERROR).json({
        message: 'internal server error'
      })
    }
  }
}
