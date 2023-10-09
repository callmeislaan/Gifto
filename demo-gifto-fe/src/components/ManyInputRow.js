import './InputRow.css';

export default function InputRow({ name, value, onAdd, deleteItem, account, setAccount }) {

    return (
        <table className="input-row">
            <tr>
                <td>
                    <span className='input-row-name'>{name}</span>
                </td>
                <td className='input-row-add'>
                    <input className='input multi-input' type="text" onChange={(e) => setAccount(e.target.value)} value={account} />
                    <button className='add-item' onClick={(e) => onAdd(e, account)}>+</button>
                </td>
                <div className='items'>
                        <ul>
                            {value.map((item) =>
                                <li key={item} className='item-element'>
                                        <span className='item-value'>{item}</span>
                                        <button className='remove-item' onClick={(e) => deleteItem(e, item)}>-</button>
                                </li>
                            )}
                        </ul>
                    </div>
            </tr>

        </table>
    )
}