import axios from 'axios';
import { useState } from 'preact/hooks';

function ToDoItem(props) {
    const [todo, setTodo] = useState({
        title: this.props.title,
        status: this.props.status,
        button: processStatus(this.props.status),
    });

    function processStatus(status) {
        if (status === 'PENDING') {
            return 'edit';
        } else {
            return 'delete';
        }
    }

    function inverseStatus(status) {
        if (status === 'PENDING') {
            return 'DONE';
        } else {
            return 'PENDING';
        }
    }

    function sendRequest() {
        axios
            .post(
                'http://127.0.0.1:8080/v1/item/' + todo.button,
                {
                    title: todo.title,
                    status: inverseStatus(todo.status),
                },
                {
                    headers: { token: 'some_token' },
                }
            )
            .then((response) => {
                this.props.passBackResponse(response);
            });
    }

    return (
        <div>
            <p>{todo.title}</p>
            <button onClick={sendRequest}>{todo.button}</button>
        </div>
    );
}

export default ToDoItem;
