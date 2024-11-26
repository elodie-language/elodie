test('A test can contain multiple descriptions'){
    describe('Hi, I am cription - des cription'){ }
    describe('Hi, I am cription - des cription too'){ }
    describe('Hi, I am cription - des cription too too'){ }
    describe('Hi, I am cription - des cription too too too'){ }
}

// out:Test: A test can contain multiple descriptions
// out:  Describe: Hi, I am cription - des cription
// out:  Describe: Hi, I am cription - des cription too
// out:  Describe: Hi, I am cription - des cription too too
// out:  Describe: Hi, I am cription - des cription too too too
