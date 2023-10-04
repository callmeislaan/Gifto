import './RegisterBrandForm.css';
import InputBanner from './InputBanner';
import InputRow from './InputRow';
import Button from './Button';
import { useState } from 'react';


export default function InputForm() {
    const [symbol, setSymbol] = useState(null);
    const [name, setName] = useState(null);
    const [avatar, setAvatar] = useState(null);
    const [description, setDescription] = useState(null);
    const [domain, setDomain] = useState(null);

    function handleCancel() {
    }

    function handleReset() {
        setSymbol(null);
        setName(null);
        setAvatar(null);
        setDescription(null);
        setDomain(null);
    }

    function handleCreate() {
        console.log(symbol);
        console.log(name);
        console.log(avatar);
        console.log(description);
        console.log(domain);
    }


    return (
        <form onSubmit={handleCreate}>
            <table>
                <tr>
                    <InputBanner value="Register Brand" />
                </tr>
                <tr>
                    <InputRow name="Symbol" onChange={setSymbol} />
                </tr>
                <tr>
                    <InputRow name="Name" onChange={setName} />
                </tr>
                <tr>
                    <InputRow name="Avatar" onChange={setAvatar} />
                </tr>
                <tr>
                    <InputRow name="Description" onChange={setDescription} />
                </tr>
                <tr>
                    <InputRow name="Domain" onChange={setDomain} />
                </tr>
                <tr>
                    <td>
                        <Button value="Cancel" onClick={handleCancel} />
                    </td>
                    <td>
                        <Button value="Reset" onClick={handleReset} />
                    </td>
                    <td>
                        <Button value="Create" onClick={handleCreate} />
                    </td>
                </tr>
            </table>
        </form>
    )
}