import express from "express"
import prisma from "./prisma" // importing the prisma instance we created.
import { graphqlHTTP } from 'express-graphql';
import { makeExecutableSchema } from '@graphql-tools/schema';
import path from 'path';
import axios from 'axios';

type Cat = {
    id: number,
    name: string
}


type Categories = {
    trivia_categories: [Cat]
}



const app = express()
app.use(express.json())

const PORT = process.env.PORT || 3000



const typeDefs = `
  type User {
    name: String!
  }

  type Categories {
    id: Int!
    name: String!
  }

  type Questions {
    category: String!
    difficulty: String!
    question: String!
    correct_answer: String!
    incorrect_answers: [String!]!
  }
  
  type Query {
    allUsers: [User!]!
    allCategories: [Categories!]!
    allQuestions: [Questions!]!
    questionsByAmount(amount: Int!): [Questions!]!
    questionsByCategoryId(category_id: Int!): [Questions!]!
    questionsByAmountAndCategoryId(amount: Int!, category_id: Int, difficulty: String, quiz_type: String): [Questions!]!
  }

`;

const resolvers = {
    Query: {
        allUsers: () => {
            return prisma.user.findMany();
        },
        allCategories: async () => {
            const response = await axios.get('https://opentdb.com/api_category.php');
            const categories = await response.data;
            return (categories as Categories).trivia_categories;
        },
        allQuestions: async () => {
            const response = await axios.get('https://opentdb.com/api.php');
            const categories = await response.data;
            return categories.results;
        },
        questionsByAmount: async (_: any, args: { amount: number; }) => {
            const response = await axios.get('https://opentdb.com/api.php?amount=' + args.amount);
            const categories = await response.data;
            return categories.results;
        },
        questionsByCategoryId: async (_: any, args: { category_id: number; }) => {
            const response = await axios.get('https://opentdb.com/api.php?amount=10&category=' + args.category_id);
            const categories = await response.data;
            return categories.results;
        },
        questionsByAmountAndCategoryId: async (_: any, args: { amount: number; category_id: number; difficulty: string; quiz_type: string }) => {
            let url = "";
            if (args.difficulty === "any" && args.quiz_type === "any" && args.category_id !== 0) {
                url = 'https://opentdb.com/api.php?amount=' + args.amount + '&category=' + args.category_id;
            } else if (args.difficulty !== "any" && args.category_id === 0 && args.quiz_type === "any") {
                url = 'https://opentdb.com/api.php?amount=' + args.amount + '&difficulty=' + args.difficulty;
            } else if (args.difficulty !== "any" && args.category_id !== 0 && args.quiz_type === "any") {
                url = 'https://opentdb.com/api.php?amount=' + args.amount + '&category=' + args.category_id + '&difficulty=' + args.difficulty;
            } else if (args.difficulty === "any" && args.category_id === 0 && args.quiz_type !== "any") {
                url = 'https://opentdb.com/api.php?amount=' + args.amount + '&type=' + args.quiz_type;
            } else if (args.difficulty === "any" && args.category_id !== 0 && args.quiz_type !== "any") {
                url = 'https://opentdb.com/api.php?amount=' + args.amount + '&category=' + args.category_id + '&type=' + args.quiz_type;
            } else if (args.difficulty !== "any" && args.category_id === 0 && args.quiz_type !== "any") {
                url = 'https://opentdb.com/api.php?amount=' + args.amount + '&difficulty=' + args.difficulty + '&type=' + args.quiz_type;
            } else if (args.difficulty !== "any" && args.category_id !== 0 && args.quiz_type !== "any") {
                url = 'https://opentdb.com/api.php?amount=' + args.amount + '&category=' + args.category_id + '&difficulty=' + args.difficulty + '&type=' + args.quiz_type;
            } else {
                url = 'https://opentdb.com/api.php?amount=' + args.amount;
            }
            const response = await axios.get(url);
            const categories = await response.data;
            return categories.results;
        }
    }
};

export const schema = makeExecutableSchema({
    resolvers,
    typeDefs,
});

app.use(express.static('../dist'));

app.get('/', function(req, res) {
  res.sendFile(path.join(__dirname, '../dist/index.html'));
});

app.use('/graphql', graphqlHTTP({
    schema,
    graphiql: true,
}));

app.listen(PORT, () => console.log(`Server is running on port ${PORT}`))

export default app;