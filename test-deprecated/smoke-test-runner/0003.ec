test('A test can contain multiple descriptions'){
    describe('Hi, I am cription - des cription'){ }
    describe('Hi, I am cription - des cription too'){ }
    describe('Hi, I am cription - des cription too too'){ }
    describe('Hi, I am cription - des cription too too too'){ }
}

// out: A test can contain multiple descriptions
// out:   Hi, I am cription - des cription
// out:   Hi, I am cription - des cription too
// out:   Hi, I am cription - des cription too too
// out:   Hi, I am cription - des cription too too too
