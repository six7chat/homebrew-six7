class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.27"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.27"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.27/six7-0.7.27-arm64-apple-darwin.tar.gz"
      sha256 "87a49efe2d9b10df33efbdb06c1c1aadb1b492fcfdaa9cfd1069f76a9b4907ab"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
