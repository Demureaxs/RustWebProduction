import { useEffect, useState } from 'preact/hooks';
import preactLogo from './assets/preact.svg';
import viteLogo from '/vite.svg';
import './app.css';

import axios from 'axios';

import ToDoItem from './components/ToDoItem';
import CreateToDoItem from './components/CreateTodoItem';

export function App() {
    const [todos, setTodos] = useState({
        pending_items: [],
        done_items: [],
        pending_item_count: 0,
        done_item_count: 0,
    });

    function getItems() {
        axios
            .get('http://127.0.0.1:8080/v1/item/get', {
                headers: {
                    token: 'some_token',
                },
            })
            .then((response) => {
                let pending_items = response.data['pending_items'];
                let done_items = response.data['done_items'];
                let pending_items_count = response.data['pending_item_count'];
                let done_items_count = response.data['done_item_count'];

                setTodos((prev) => ({
                    ...prev,
                    pending_items,
                    done_items,
                    pending_items_count,
                    done_items_count,
                }));
            });
    }

    useEffect(() => {
        getItems();
    }, []);

    function processItemValues(items) {
        let itemList = [];
        items.forEach((item, _) => {
            itemList.push(
                <ToDoItem
                    key={item.title + item.status}
                    title={item.title}
                    status={item.status.status}
                    passBackResponse={handleReturnedState}
                />
            );
        });
        return itemList;
    }

    function handleReturnedState(response) {
        let pending_items = response.data['pending_items'];
        let done_items = response.data['done_items'];
        setTodos({
            pending_items: processItemValues(pending_items),
            done_items: processItemValues(done_items),
            pending_item_count: response.data['pending_item_count'],
            done_item_count: response.data['done_item_count'],
        });
    }

    return (
        <div className='App'>
            <h1>Pending Items</h1>
            <p>Done Item Count: {todos.pending_item_count}</p>
            {todos.pending_items}
            <h1>Done Items: </h1>
            <p>Done Item Count: {todos.done_item_count}</p>
            {todos.done_items}
            <CreateToDoItem passBackResponse={handleReturnedState} />
        </div>
    );
}
