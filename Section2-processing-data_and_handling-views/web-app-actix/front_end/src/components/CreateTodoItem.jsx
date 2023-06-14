import axios from 'axios';
import { useState } from 'preact/hooks';

function CreateToDoItem() {
    const [todo, setTodo] = useState({
        title: '',
    });

    function createItem() {
        axios
            .post(
                'http://127.0.0.1:8080/v1/item/create' + todo.title,
                {},
                {
                    headers: { token: 'some_token' },
                }
            )
            .then((response) => {
                setTodo({ title: '' });
                this.props.passBackResponse(response);
            });
    }

    function handleTitleChange() {
        setTodo({ title: e.target.value });
    }

    return (
        <div className='inputContainer'>
            <input
                type='text'
                id='name'
                placeholder='Create to do item'
                value={todo.title}
                onChange={handleTitleChange}
            />
            <div
                className='actionButton'
                id='create-button'
                onClick={createItem}
            ></div>
        </div>
    );
}

export default CreateToDoItem;
