import React from 'react';
import styled from 'styled-components'
import './App.css';

export function App(): React.ReactElement {
  const [width, height] = [100, 100];
  const interval = 100;
  const [field, setField] = React.useState<Uint8Array>(null as any);

  React.useEffect(() => {
    (async () => {
      const lifegame = await import('lifegame/lifegame');
      const { LifeGame, Cell } = lifegame;
      const game = LifeGame.new(width, height);
      for (let x = 0; x < width; x++) {
        for (let y = 0; y < height; y++) {
          game.set_cell(x, y, Math.random() >= 0.5 ? Cell.Alive : Cell.Dead);
        }
      }

      const buf = new Uint8Array(width * height);
      game.fill_cells(buf);
      setField(buf);

      setInterval(() => {
        game.next();

        const buf = new Uint8Array(width * height);
        game.fill_cells(buf);
        setField(buf);
      }, interval);
    })();
  }, [width, height]);

  return (
    <div className="App">
      <LifeGameField width={width} height={height} field={field} />
    </div>
  );
}

function LifeGameField(props: { width: number, height: number, field: Uint8Array }): React.ReactElement {
  const { width, height, field } = props;
  if (!field) return null as any;

  return <Field>
    {
      Array.from(Array(height).keys()).map(y => (
        <Row>
          {
            Array.from(Array(width).keys()).map(x => (
              field[y * width + x] ? (<Alive />) : (<Dead />)
            ))
          }
        </Row>
      ))
    }
  </Field>
}

const Field = styled.table`
  margin: 10px;
  border-spacing:0;
`;

const Row = styled.tr`
`;

const Cell = styled.td`
  width:3px;
  height:3px;
  padding:0;
`

const Alive = styled(Cell)`
  background-color: rgb(0,255,0);
`;

const Dead = styled(Cell)`
  background-color: rgb(255,0,255);
`;