interface Props {
  code: number;
}

const Error: React.FC<Props> = function (props) {
  return <p>{props.code}</p>;
};

export default Error;
