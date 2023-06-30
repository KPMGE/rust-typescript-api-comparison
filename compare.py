import requests
from timeit import default_timer as timer
from requests.exceptions import HTTPError

AMOUNT_USERS = 80
AMOUNT_TODOS = 80
AMOUNT_TESTS = 10

def test_whole_execution(amount_tests):
    results = []

    for _ in range(amount_tests):
        start = timer()

        create_users(AMOUNT_USERS)
        users = list_users()
        update_users(users)

        create_todos(users, AMOUNT_TODOS)

        all_todos = list_todos(users)

        update_todos(all_todos)

        delete_todos(all_todos)
        delete_users(users)

        end = timer()
        results.append(end - start)

    return results, sum(results) / len(results)

def main():
    results, average = test_whole_execution(AMOUNT_TESTS)

    print('results: [')
    for result in results:
        print(f'{result}')
    print(']')

    print(f'average: {average}')

    print(f'amount users: {AMOUNT_USERS}')
    print(f'amount todos for user: {AMOUNT_TODOS}')



def create_users(amount): 
    for c in range(amount): 
        try:
            requests.post('http://localhost:3333/users', json={
                'name': f'user {c}',
                'email': f'user{c}@gmail.com',
                'password': f'password{c}'
            })

        except HTTPError as http_err:
            print(f'HTTP error occurred: {http_err}')  
        except Exception as err:
            print(f'Other error occurred: {err}')  

def update_users(users):
    for user in users: 
        try:
            requests.put(f'http://localhost:3333/users/{user["id"]}', json={
                'name': f'updated user {user["id"]}',
                'email': f'updateduser{user["id"]}@gmail.com',
            })

        except HTTPError as http_err:
            print(f'HTTP error occurred: {http_err}')  
        except Exception as err:
            print(f'Other error occurred: {err}')  

def list_users():
    try:
        return requests.get('http://localhost:3333/users').json()
    except HTTPError as http_err:
        print(f'HTTP error occurred: {http_err}')  # Python 3.6
    except Exception as err:
        print(f'Other error occurred: {err}')  # Python 3.6

def delete_users(users):
    for user in users: 
        try:
            requests.delete(f'http://localhost:3333/users/{user["id"]}')
        except HTTPError as http_err:
            print(f'HTTP error occurred: {http_err}')  # Python 3.6
        except Exception as err:
            print(f'Other error occurred: {err}')  # Python 3.6


def create_todos(users, amount_todos): 
    for user in users: 
        for k in range(amount_todos):
            try:
                requests.post(f'http://localhost:3333/todos/{user["id"]}', json={
                    'title': f'todo {k} for user {user["id"]}',
                    'description': f'description {k} todo for user {user["id"]}',
                })

            except HTTPError as http_err:
                print(f'HTTP error occurred: {http_err}')  # Python 3.6
            except Exception as err:
                print(f'Other error occurred: {err}')  # Python 3.6

def list_todos(users): 
    all_todos = []

    for user in users: 
        try:
            todos = requests.get(f'http://localhost:3333/todos/{user["id"]}').json()
            all_todos = all_todos + todos
        except HTTPError as http_err:
            print(f'HTTP error occurred: {http_err}')
        except Exception as err:
            print(f'Other error occurred: {err}')
            
    return all_todos

def update_todos(all_todos):
    for todo in all_todos: 
        try:
            requests.put(f'http://localhost:3333/todos/{todo["id"]}', json={
                'title': f'updated todo {todo["id"]}',
                'description': f'updated description {todo["id"]}',
            })

        except HTTPError as http_err:
            print(f'HTTP error occurred: {http_err}')
        except Exception as err:
            print(f'Other error occurred: {err}')

def delete_todos(todos):
    for todo in todos: 
        try:
            requests.delete(f'http://localhost:3333/todos/{todo["id"]}')
        except HTTPError as http_err:
            print(f'HTTP error occurred: {http_err}')
        except Exception as err:
            print(f'Other error occurred: {err}')


if __name__ == "__main__":
    main()
