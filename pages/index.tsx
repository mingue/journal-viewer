import { invoke } from '@tauri-apps/api/tauri'
import { useEffect, useState } from 'react'

interface JournalEntries {
  headers: Array<string>,
  rows: Array<Array<string>>;
}

export default function Home() {
  const [rowCount, setRowCount] = useState(0);
  const [rows, setRows] = useState<JournalEntries>({ headers: [], rows: [] } as JournalEntries);
  useEffect(() => {
    const fetchData = async () => {
      let logsStr: string = await invoke('get_logs');
      let logs: JournalEntries = JSON.parse(logsStr);
      setRows(logs);
      setRowCount(logs.rows.length);
    }
    fetchData();
  }, []);

  return (
    <div className="container-fluid">
      <div className="row">
        <div className="col-6">
          <h2>System Logs</h2>
        </div>
        <div className="col">
          <input type="text" placeholder='Quick Filter' className="form-control" id="quick"></input>
        </div>
        <div className="col-1">
          <i className="bi bi-question-circle"></i>
        </div>
      </div>
      <div className="row">
        <div className="col">
          Logs chart by priority and broken down by time like 24h/15m
          <input type="text" className="form-control" id="unit"></input>
        </div>
      </div>
      <div className="row">
        <div className="col-3">
          <form>
            <div className="row">
              <div className="col-12">
                <label htmlFor="unit" className="form-label">Unit</label>
                <input type="text" className="form-control" id="unit"></input>
              </div>
              <div className="col-12">
                <label htmlFor="boot" className="form-label">Boot</label>
                <input type="text" className="form-control" id="boot"></input>
              </div>
            </div>
          </form>
        </div>
        <div className="col">
          <div className="row">
            {rows.headers.map(h => {
              return (
                <div className="col">
                  {h}
                </div>
              );
            })}
          </div>
          {rows.rows.map(r => {
            return (
              <div className="row">
                {r.map(f => {
                  return (
                    <div className="col">
                      {f.substring(0, f.length > 30 ? 30 : f.length)}
                    </div>
                  );
                })}
              </div>
            );
          })}
          I have found this many logs {rowCount}
        </div>
      </div>
    </div >
  )
}
