import validator from 'validator'
import { User } from '../../domain/entities'
import { CheckUserRepository, CreateUserRepository } from "../repositories";
import { 
  DuplicateUserNameError,
  InvalidEmailError, 
  InvalidPasswordError,
  InvalidUserNameLenghtError
} from '../../domain/errors';

const minPasswordLength = 5
const minUserNameLength = 5

export class CreateUserService {
  constructor(
    private readonly createUserRepo: CreateUserRepository,
    private readonly checkUserRepo: CheckUserRepository
  ) { }

  async create(user: User): Promise<void> {
    if (!validator.isEmail(user.email)) {
      throw new InvalidEmailError(user.email)
    }

    if (user.password.length < minPasswordLength) {
      throw new InvalidPasswordError()
    }

    if (user.name.length < minUserNameLength) {
      throw new InvalidUserNameLenghtError(minUserNameLength)
    }

    const userExists = await this.checkUserRepo.exists(user)
    if (userExists) {
      throw new DuplicateUserNameError(user.name)
    }
 
    await this.createUserRepo.create(user)
  }
}
