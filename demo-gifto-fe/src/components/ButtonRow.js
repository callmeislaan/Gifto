import Button from './Button'
import './ButtonRow.css'

export default function ButtonRow({value1, value2, value3, handle1, handle2, handle3}) {
    return (
        <div className='button-row'>
            <Button value={value1} onClick={(e) => handle1(e)} />
            <Button value={value2} onClick={(e) => handle2(e)} />
            <Button value={value3} onClick={(e) => handle3(e)} />
        </div>
    )
}