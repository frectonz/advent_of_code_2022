module Main exposing (..)

import Html exposing (Html, div)
import Parser exposing ((|.), (|=), Parser, getChompedString, spaces, succeed)
import Test


type GameMove
    = Rock
    | Paper
    | Scissors
    | UnknownMove


type GameEnd
    = Win
    | Lose
    | Draw
    | UnknownEnd


type alias Line =
    { firstStr : String
    , secondStr : String
    }


type alias Part1Input =
    { opponentMove : GameMove
    , myMove : GameMove
    }


type alias Part2Input =
    { opponentMove : GameMove
    , gameEnd : GameEnd
    }


parseABCToMove : String -> GameMove
parseABCToMove move =
    case move of
        "A" ->
            Rock

        "B" ->
            Paper

        "C" ->
            Scissors

        _ ->
            UnknownMove


parseXYZToMove : String -> GameMove
parseXYZToMove move =
    case move of
        "X" ->
            Rock

        "Y" ->
            Paper

        "Z" ->
            Scissors

        _ ->
            UnknownMove


parseXYZToGameEnd : String -> GameEnd
parseXYZToGameEnd gameEnd =
    case gameEnd of
        "X" ->
            Lose

        "Y" ->
            Draw

        "Z" ->
            Win

        _ ->
            UnknownEnd


parseLine : Parser Line
parseLine =
    succeed Line
        |= getChompedString (Parser.chompWhile (\c -> c /= ' '))
        |. spaces
        |= getChompedString (Parser.chompWhile (\c -> c /= ' '))


lineToPart1Input : Line -> Part1Input
lineToPart1Input line =
    { opponentMove = parseABCToMove line.firstStr
    , myMove = parseXYZToMove line.secondStr
    }


lineToPart2Input : Line -> Part2Input
lineToPart2Input line =
    { opponentMove = parseABCToMove line.firstStr
    , gameEnd = parseXYZToGameEnd line.secondStr
    }


calculateGame : Part1Input -> Int
calculateGame { opponentMove, myMove } =
    case ( myMove, opponentMove ) of
        ( Rock, Scissors ) ->
            6 + 1

        ( Rock, Paper ) ->
            0 + 1

        ( Rock, Rock ) ->
            3 + 1

        ( Paper, Rock ) ->
            6 + 2

        ( Paper, Scissors ) ->
            0 + 2

        ( Paper, Paper ) ->
            3 + 2

        ( Scissors, Paper ) ->
            6 + 3

        ( Scissors, Rock ) ->
            0 + 3

        ( Scissors, Scissors ) ->
            3 + 3

        ( _, _ ) ->
            0


calculateMove : Part2Input -> Part1Input
calculateMove { opponentMove, gameEnd } =
    let
        myMove =
            case ( opponentMove, gameEnd ) of
                ( Rock, Win ) ->
                    Paper

                ( Rock, Draw ) ->
                    Rock

                ( Rock, Lose ) ->
                    Scissors

                ( Paper, Win ) ->
                    Scissors

                ( Paper, Draw ) ->
                    Paper

                ( Paper, Lose ) ->
                    Rock

                ( Scissors, Win ) ->
                    Rock

                ( Scissors, Draw ) ->
                    Scissors

                ( Scissors, Lose ) ->
                    Paper

                ( _, _ ) ->
                    UnknownMove
    in
    { opponentMove = opponentMove, myMove = myMove }


part1Answer : String -> Int
part1Answer s =
    s
        |> String.lines
        |> List.map
            (\line ->
                Parser.run parseLine line
                    |> Result.map lineToPart1Input
                    |> Result.map calculateGame
                    |> Result.withDefault 0
            )
        |> List.foldl (+) 0


part2Answer : String -> Int
part2Answer s =
    s
        |> String.lines
        |> List.map
            (\line ->
                Parser.run parseLine line
                    |> Result.map lineToPart2Input
                    |> Result.map calculateMove
                    |> Result.map calculateGame
                    |> Result.withDefault 0
            )
        |> List.foldl (+) 0


main : Html a
main =
    div []
        [ Test.mainInput
            |> part1Answer
            |> String.fromInt
            |> (++) "Part 1 Solution: "
            |> Html.text
        , Html.br [] []
        , Test.mainInput
            |> part2Answer
            |> String.fromInt
            |> (++) "Part 2 Solution: "
            |> Html.text
        ]
